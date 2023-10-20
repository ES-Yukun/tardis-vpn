# Tardis VPN
Tardis is a high-speed VPN (not encrypted) for server-to-server communication.

## Usage
- **Install Tardis (Ubuntu)**
```bash
sudo apt install -y cargo make
git clone https://github.com/docheio/tardis.git
cd tardis
make all
make install
```

- **Update Tardis**
```bash
make upgrade
```

- **Tardis command usage**
```bash
# Peer mode [peer-to-peer]
tardis peer <LISTEN-IP>:<PORT> <TARGET-IP>:<PORT> <INTERFACE-NAME> <INTERFACE-IP>/<IP-RANGE>

# Server mode [server-to-client]
tardis server <LISTEN-IP>:<PORT> <INTERFACE-NAME> <INTERFACE-IP>/<IP-RANGE>

# Client mode [client-to-server]
tardis client <TARGET-IP>:<PORT> <INTERFACE-NAME> <INTERFACE-IP>/<IP-RANGE>
```
- **Start Service**
```bash
# Must edit config „/etc/systemd/system/tardisd.service“
systemctl enable --now tardisd
```
