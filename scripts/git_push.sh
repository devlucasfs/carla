PROJECT="Carla"

echo git add .
git add .

echo git commit -m \"$1\"
git commit -m "$1"

echo git push origin main
git push origin main

echo

echo git clone git@github.com:devlucasfs/uplogs.git
git clone git@github.com:devlucasfs/uplogs.git

echo cd uplogs
cd uplogs

cat README.md | { cat; echo "\n# New commit in $PROJECT.\n\`\`\`\n$1\n\`\`\` "; }

echo git add .
git add .

echo git commit -m \"update($PROJECT): $1\"
git commit -m \"update($PROJECT): $1\"

echo git push origin main
git push origin main
