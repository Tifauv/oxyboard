FROM gentoo/stage3:amd64-hardened-nomultilib

MAINTAINER Olivier Serve <tifauv@gmail.com>
LABEL org.opencontainers.image.authors="tifauv.gmail.com"

ENV OXYBOARD_HOME=/opt/oxyboard

RUN groupadd --gid 1042 oxyboard && \
	useradd --uid 1042 --gid oxyboard --comment 'Oxyboard service account' --home-dir "${OXYBOARD_HOME}" --create-home oxyboard && \
	chown -R oxyboard:oxyboard "${OXYBOARD_HOME}" && \
	mkdir -pv "${OXYBOARD_HOME}/bin" && \
	mkdir -pv "${OXYBOARD_HOME}/data"

USER oxyboard
WORKDIR ${OXYBOARD_HOME}

COPY target/debug/oxyboard ./bin
COPY config    ./config
COPY templates ./templates

# Default port is 8080/tcp
EXPOSE 8080

# Data is stored in a "data" directory
VOLUME ${OXYBOARD_HOME}/data

STOPSIGNAL SIGINT
ENTRYPOINT ["/opt/oxyboard/bin/oxyboard"]
