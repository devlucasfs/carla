// gosched_asm.c — igual ao gosched.c, mas a troca de contexto é assembly puro
// (labels + push/pop/ret), sem ucontext. Somente x86_64 System V (Linux/GNU).
//
// Compilar: gcc -O2 -Wall -o gosched_asm gosched_asm.c && ./gosched_asm

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define STACK_SIZE (64 * 1024)


__asm__(
    ".text\n"
    ".globl ctx_switch\n"
    "ctx_switch:\n"
    "    pushq %rbp\n"
    "    pushq %rbx\n"
    "    pushq %r12\n"
    "    pushq %r13\n"
    "    pushq %r14\n"
    "    pushq %r15\n"
    "    movq  %rsp, (%rdi)\n"
    "    movq  %rsi, %rsp\n"
    "    popq  %r15\n"
    "    popq  %r14\n"
    "    popq  %r13\n"
    "    popq  %r12\n"
    "    popq  %rbx\n"
    "    popq  %rbp\n"
    "    ret\n"
);
void ctx_switch(void **save_sp, void *load_sp);

/* ---------- goroutines ---------- */

typedef enum { G_RUNNABLE, G_RUNNING, G_WAITING, G_DEAD } GState;

typedef struct G {
    void *sp;         // rsp salvo quando a goroutine não está rodando
    void (*fn)(void *);
    void *arg;
    GState state;
    char *stack;
    struct G *next;   // ligação nas filas
    int id;
} G;

typedef struct { G *head, *tail; } GQueue;

static void *sched_sp;         // rsp salvo do scheduler
static G *current;             // goroutine rodando agora
static GQueue runq;            // fila de prontas
static int live;               // goroutines vivas
static int next_id = 1;

static void gq_push(GQueue *q, G *g) {
    g->next = NULL;
    if (q->tail) q->tail->next = g; else q->head = g;
    q->tail = g;
}

static G *gq_pop(GQueue *q) {
    G *g = q->head;
    if (g) {
        q->head = g->next;
        if (!q->head) q->tail = NULL;
    }
    return g;
}

static void ready(G *g) {
    g->state = G_RUNNABLE;
    gq_push(&runq, g);
}

// Devolve o controle ao scheduler (a goroutine já deve estar numa fila)
static void switch_to_sched(void) {
    ctx_switch(&current->sp, sched_sp);
}

// Equivalente a runtime.Gosched()
void yield(void) {
    ready(current);
    switch_to_sched();
}

// Bloqueia a goroutine atual até alguém chamar ready() nela
static void park(void) {
    current->state = G_WAITING;
    switch_to_sched();
}

static void g_entry(void) {
    current->fn(current->arg);
    current->state = G_DEAD;
    live--;
    switch_to_sched();   // nunca volta
}

// go(fn, arg)  ~  go fn(arg)
void go(void (*fn)(void *), void *arg) {
    G *g = calloc(1, sizeof *g);
    g->fn = fn;
    g->arg = arg;
    g->id = next_id++;
    g->stack = malloc(STACK_SIZE);
    // Monta uma stack "fingindo" que a goroutine já foi suspensa por ctx_switch:
    //   [r15 r14 r13 r12 rbx rbp][&g_entry][0 = endereço de retorno falso]
    uintptr_t top = ((uintptr_t)g->stack + STACK_SIZE) & ~(uintptr_t)15;
    void **sp = (void **)top;
    *--sp = NULL;              // alinhamento: no 1º instante de g_entry, rsp % 16 == 8
    *--sp = (void *)g_entry;   // o `ret` do ctx_switch pula para cá
    for (int i = 0; i < 6; i++) *--sp = NULL;   // rbp rbx r12 r13 r14 r15
    g->sp = sp;
    live++;
    ready(g);
}

// Loop do scheduler: roda até não haver mais nada pronto
void sched_run(void) {
    G *g;
    while ((g = gq_pop(&runq))) {
        current = g;
        g->state = G_RUNNING;
        ctx_switch(&sched_sp, g->sp);
        if (g->state == G_DEAD) {
            free(g->stack);
            free(g);
        }
    }
    if (live > 0) {
        fprintf(stderr, "fatal error: all goroutines are asleep - deadlock!\n");
        exit(2);
    }
}

/* ---------- canais (bufferizados, cap >= 1) ---------- */

typedef struct {
    int *buf;
    int cap, len, head;
    int closed;
    GQueue recvq, sendq;   // goroutines bloqueadas
} Chan;

Chan *chan_make(int cap) {
    Chan *c = calloc(1, sizeof *c);
    c->cap = cap;
    c->buf = malloc(sizeof(int) * cap);
    return c;
}

static void wake_one(GQueue *q) {
    G *g = gq_pop(q);
    if (g) ready(g);
}

void chan_send(Chan *c, int v) {
    for (;;) {
        if (c->closed) { fprintf(stderr, "panic: send on closed channel\n"); abort(); }
        if (c->len < c->cap) {
            c->buf[(c->head + c->len) % c->cap] = v;
            c->len++;
            wake_one(&c->recvq);
            return;
        }
        gq_push(&c->sendq, current);   // cheio: dorme
        park();
    }
}

// retorna 0 se o canal está fechado e vazio (como `v, ok := <-ch`)
int chan_recv(Chan *c, int *out) {
    for (;;) {
        if (c->len > 0) {
            *out = c->buf[c->head];
            c->head = (c->head + 1) % c->cap;
            c->len--;
            wake_one(&c->sendq);
            return 1;
        }
        if (c->closed) return 0;
        gq_push(&c->recvq, current);   // vazio: dorme
        park();
    }
}

void chan_close(Chan *c) {
    c->closed = 1;
    G *g;
    while ((g = gq_pop(&c->recvq))) ready(g);
    while ((g = gq_pop(&c->sendq))) ready(g);
}

/* ---------- demo: worker pool ---------- */

#define NJOBS    10
#define NWORKERS 3

static Chan *jobs, *results;
static int workers_alive = NWORKERS;

static void producer(void *arg) {
    (void)arg;
    for (int i = 1; i <= NJOBS; i++) {
        printf("[producer ] enviando job %d\n", i);
        chan_send(jobs, i);
    }
    chan_close(jobs);
}

static void worker(void *arg) {
    long id = (long)arg;
    int job;
    while (chan_recv(jobs, &job)) {
        printf("[worker %ld ] processando job %d\n", id, job);
        yield();                          // simula trabalho / cede a vez
        chan_send(results, job * job);
    }
    if (--workers_alive == 0) chan_close(results);
}

static void main_goroutine(void *arg) {
    (void)arg;
    go(producer, NULL);
    for (long i = 1; i <= NWORKERS; i++) go(worker, (void *)i);

    int r, sum = 0;
    while (chan_recv(results, &r)) {
        printf("[main     ] resultado: %d\n", r);
        sum += r;
    }
    printf("[main     ] soma dos quadrados = %d\n", sum);
}

int main(void) {
    jobs    = chan_make(2);
    results = chan_make(2);
    go(main_goroutine, NULL);
    sched_run();
    return 0;
}
