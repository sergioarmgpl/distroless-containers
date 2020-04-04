FROM ubuntu:18.04 as build

RUN apt-get update && apt-get install \
  -y --no-install-recommends python3 python3-virtualenv

RUN python3 -m virtualenv --python=/usr/bin/python3 /opt/venv

COPY requirements.txt .
RUN . /opt/venv/bin/activate && pip install -r requirements.txt
RUN apt-get autoremove -y python3-pip
RUN apt-get clean
RUN apt-get purge


COPY index.py .
CMD . /opt/venv/bin/activate && exec python index.py
