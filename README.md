# server

### setup
```
git https://github.com/2LJDC/server
```
```
cd server
```
```
https://github.com/2LJDC/Website
```
```
mv Website/ www/
```
```
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```
```
cargo build --release
```
```
podman build -t 2ljdc-server .
```
```
podman run -dt --name 2ljdc -p 0.0.0.0:80:8000 2ljdc-server
```


### check database
```
PGPASSWORD="key" psql -U postgres -h 127.0.0.1 -p 5432
```
```
SELECT * FROM kunde;
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
### sneaky way for more mem on vServer
```
dd if=/dev/zero of=/swapfile bs=1M count=8192 
```
```
chmod 0600 /swapfile
```
```
mkswap /swapfile 
```
```
swapon /swapfile 
```
/etc/fstab 
```
/swapfile         none           swap     sw                       0    0
```
