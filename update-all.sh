

git subtree push --prefix=randy-gateway gateway main
git subtree push --prefix=randy-model model main
git subtree push --prefix=randy-ratelimiting ratelimiting main
git subtree push --prefix=randy-rest rest main
git subtree push --prefix=randy-storage storage main
git subtree push --prefix=randy-tools tools main
git subtree push --prefix=randy-validate validate main
git push -u origin main
