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
bash build.sh
```
```
sed...
```
```
docker build -t 2ljdc-server .
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
### certbot
```
dnf install -y epel-release
```
```
dnf install -y snapd
```
```
systemctl enable --now snapd.socket
```
```
ln -s /var/lib/snapd/snap /snap
```
```
snap install --classic certbot
```
```
ln -s /snap/bin/certbot /usr/bin/certbot
```
