FROM debian:bookworm-slim
RUN apt-get update && apt-get install 
COPY .local/node-template /substrate/
WORKDIR /substrate
RUN ls -l
CMD ["/substrate/node-template", "--dev", "--ws-external", "--rpc-external"]