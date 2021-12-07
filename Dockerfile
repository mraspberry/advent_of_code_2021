FROM rust:1.56

WORKDIR /code

RUN addgroup --system builder && \
    adduser --ingroup builder builder && \
    rustup component add rustfmt

COPY --chown=builder:builder . /code 

USER builder

CMD ["/code/bin/build-and-verify.sh"]
