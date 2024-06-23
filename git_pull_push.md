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

################################
cd D:\Project\rust_learning\module_sys\src

# 创建文件
New-Item -ItemType File math.rs
New-Item -ItemType File network.rs

# 添加内容到 math.rs
Set-Content -Path math.rs -Value "pub fn add(a: i32, b: i32) -> i32 {
    a + b
}"

# 添加内容到 network.rs
Set-Content -Path network.rs -Value "pub fn connect() {
    println!(\"Connected to network.\");
}"

# 添加内容到 main.rs
Set-Content -Path main.rs -Value "mod math;
mod network;

fn main() {
    let result = math::add(5, 3);
    println!(\"5 + 3 = {}\", result);
    
    network::connect();
}"

# 返回项目根目录并运行
cd ..
cargo run
