# messenger

~~LINEのパクリです。~~

外部依存なしで実行できるメッセージアプリです。

ビルドには,

- cargo

が必要ですが、ローカルで動かすだけなら`docker compose up`で動くと思います。

---
進捗

server-side
---
### db
- [x] accounts
- [x] operations
- [x] messages
- [x] profiles
- [ ] friends

### grpc
- [ ] account service
- [ ] operation service
- [ ] supervisor service
- [ ] talk service

### idgen
- [x] rest api server

### blob
- [x] rest api server

### proxy
- [x] envoy

---

client-side
---
### mobile
- [ ] android
- [ ] ios

### web
- [ ] react...?