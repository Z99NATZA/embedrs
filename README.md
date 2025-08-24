# EmbedRS
โปรเจกต์ฝึกทำ **Text Embeddings** ด้วย [async-openai](https://crates.io/crates/async-openai) + Rust พร้อมระบบทดสอบและ logging สำหรับดู performance

> เหมาะสำหรับผู้ที่ต้องการเรียนรู้การทำ Embeddings แบบเร็ว ๆ

---

## ✨ ไฮไลต์
- ⚙️ ใช้ **Rust + Tokio** ทำงานแบบ async เร็วและปลอดภัย
- 🤖 ใช้ **async-openai** เรียก Embeddings API ได้สะดวก
- 🧪 มี **unit tests** และ **integration tests** (`cargo test`)

---

## 📦 การติดตั้ง (Quick Start)
```bash
git clone https://github.com/Z99NATZA/embedrs.git
cd embedrs
cargo build
```

### ตั้งค่า Environment
เปลี่ยน `.env.example` เป็น `.env` แล้วตั้งค่า
```dotenv
OPENAI_API_KEY=ใส่คีย์
```

> หากยังไม่มีคีย์: สมัครที่ OpenAI แล้วสร้าง API key จากหน้า Dashboard

### ทดลองสร้าง Embedding
```bash
cargo run
```

---

## ⚙️ Environment Variables
ไฟล์ `.env`
```dotenv
# จำเป็น
OPENAI_API_KEY=
OPENAI_EMBED_MODEL=
```

---

## 📦 crate
- [async-openai](https://crates.io/crates/async-openai)
- ขอบคุณ ชุมชน Rust และโอเพนซอร์สทุกท่าน ❤️

---

## 🧪 TL;DR คำสั่งหลัก
```bash
# ติดตั้ง
git clone https://github.com/Z99NATZA/embedrs.git
cd embedrs
cargo build

# เปลี่ยน .env.example เป็น .env
# แล้วตั้ง OPENAI_API_KEY=...

# รันทดสอบปกติ (เร็ว)
cargo test

# รันเฉพาะเทสที่ถูก ignore (เช่น เทสที่เรียก API จริง)
cargo test -- --ignored

# รันทั้งหมด รวม ignored
cargo test -- --include-ignored

# ทดสอบสร้าง embedding
cargo run
```


