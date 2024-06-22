#首次推送
cd D:\Project\rust_learning
git init
git remote add origin https://github.com/Taylor-Tian/rust_learning.git
git branch
git checkout main
git pull origin main
git add .
git commit -m "Add new Rust projects"
git push origin main

#后续的推送步骤
git pull origin main  # 确保拉取最新的分支更新
git add .
git commit -m "Add new Rust projects"
git push origin main
