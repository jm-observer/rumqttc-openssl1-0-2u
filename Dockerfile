FROM huangjiemin/mipsel-uclibc-openssl:1.65.0_stable

WORKDIR /opt/

COPY openssl_1.0.2_uclibc.tar /opt/
RUN tar -xvf /opt/openssl_1.0.2_uclibc.tar && \
    rm -rf /opt/openssl_1.0.2_uclibc.tar && \
    cp /opt/openssl/lib/libssl.so.1.0.0 /usr/lib/ && \
    cp /opt/openssl/lib/libcrypto.so.1.0.0 /usr/lib/


ENV MIPSEL_UNKNOWN_LINUX_UCLIBC_OPENSSL_INCLUDE_DIR="/opt/openssl"
ENV MIPSEL_UNKNOWN_LINUX_UCLIBC_OPENSSL_LIB_DIR="/opt/openssl/lib"

# /usr/local/cargo/registry
WORKDIR /root/src