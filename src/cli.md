### csv转换处理
cargo run -- csv -i assets/juventus.csv
- serde_json
- serde_yaml
- clap
- csv
- serde
### 生成随机密码
- zxcvbn
- rand
### base64解编码
- base 64
### 文本签名
防止文件篡改
- aws-signature-v4
- hash库 blake3 
- 非对称加密 ed25519_dalek
- Trait Signer / Verifier
- 构建CLI 对输入文本哈希/签名
- rcli text sign -key sk.pem
- rcli text verify -key pk.pem
### 文件服务器
- **tokio** 异步支持
- **axum** web框架
- **hyper** http客户端
- **reqwest** http客户端
- **tower** 中间件
- **tonic** grpc客户端