FROM python:3.11.9-slim
ENV PYTHONUNBUFFERED=0
ENV PYTHONPATH=/app/content
ENV GRADIO_ALLOW_FLAGGING=never
ENV GRADIO_ANALYTICS_ENABLED=0
ENV GRADIO_NUM_PORTS=1
ENV GRADIO_SERVER_NAME=0.0.0.0
ENV GRADIO_SERVER_PORT=8008

# WORKDIR /gradio/certs
# RUN openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
#     -subj "/C=US/ST=California/L=Santa Clara/O=NVIDIA/OU=NIM/CN=ai.nvidia.com/" \
#     -keyout key.pem \
#     -out cert.pem

COPY app/requirements.txt /tmp/requirements.txt
RUN pip3 install -r /tmp/requirements.txt
WORKDIR /app