apt-get install python3-venv
apt-get install python3-pip 
mkdir virtualenv
cd virtualenv
python3 -m venv env
source env/bin/activate

CREO ARCHIVO CON requirements los paquetes que necesito

pip3 install -r requirements.txt
deactivate
