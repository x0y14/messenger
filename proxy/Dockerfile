#FROM envoyproxy/envoy:v1.22.0
FROM envoyproxy/envoy:v1.15.0
COPY ./envoy.yaml /etc/envoy/envoy.yaml

ENTRYPOINT [ "/usr/local/bin/envoy" ]
CMD [ "-c /etc/envoy/envoy.yaml"]