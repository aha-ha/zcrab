# zcrab
A way to decompress all of the common compressed file types, with just one simple command

# Installation
### Debian or Ubuntu based Distributions
```bash
sudo wget -O- https://zcrab-repo.github.io/public.gpg.key | gpg --dearmor | sudo tee /usr/share/keyrings/zcrab-archive-keyring.gpg > /dev/null

echo "deb [signed-by=/usr/share/keyrings/zcrab-archive-keyring.gpg] https://zcrab-repo.github.io/ stable main" | sudo tee /etc/apt/sources.list.d/zcrab.list

sudo apt update

sudo apt install zcrab
```
### Other Distribuitons
- Go to https://github.com/aha-ha/zcrab/releases
- Download zcrab (without any file extension)
- Make it executable: `chmod +x zcrab`
- Copy to path: `sudo cp zcrab /usr/local/bin`