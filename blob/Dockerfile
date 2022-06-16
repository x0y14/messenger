FROM python:3.10-slim
WORKDIR /usr/local/app

RUN apt-get update &&  apt-get -y install libmagic1
COPY ./blob/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY ./blob .

EXPOSE 4040
CMD ["python", "server.py"]