# EmbedRS
โปรเจกต์ฝึกทำ **Text Embeddings** ด้วย [async-openai](https://crates.io/crates/async-openai) + Rust
---

## ไฮไลต์
- **Rust + Tokio** ทำงานแบบ async เร็วและปลอดภัย
- **async-openai** เรียก Embeddings API ได้สะดวก
- **unit tests** และ **integration tests** (`cargo test`)

---

## การติดตั้ง Install
```bash
git clone https://github.com/Z99NATZA/embedrs.git
cd embedrs
cargo build

# .env
# OPENAI_EMBED_MODEL=
# OPENAI_API_KEY=ใส่คีย์
# หากยังไม่มีคีย์: สมัครที่ OpenAI แล้วสร้าง API key จากหน้า Dashboard

# run
cargo run
```


