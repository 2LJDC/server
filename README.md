# server
```
git clone https://github.com/CasinoMLU/server/tree/main
```
```
cd server
```
```
cargo build --release
```
```
cp src/target/server /bin/server
```

### /var/www
```
mkdir /var/www
```
```
adduser srv --shell=/bin/false --no-create-home
```

### systemd service
```
wget https://raw.githubusercontent.com/CasinoMLU/server/main/server.service
```
```
mv server.service /etc/systemd/system/server.service
```
```
systemctl start server.service
```
```
systemctl enable server.service
```
