# xenial is required due to the version of SSL that Osgood currently links against.
# xenial is supported until April 2021: https://wiki.ubuntu.com/XenialXerus/ReleaseNotes#Support_lifespan
FROM ubuntu:xenial

ENV OSGOOD_VERSION 0.2.1
ENV OSGOOD_SHA 20be5e5d78a6a92e18b03847b6249e374102580e54b502616776944842cb17f0

RUN set -eux; \
	groupadd -r -g 1000 osgood; \
	useradd -r -g osgood -u 1000 osgood; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		ca-certificates \
		wget \
		unzip \
		libssl1.0.2 \
	; \
	wget -O osgood.zip "https://github.com/IntrinsicLabs/osgood/releases/download/$OSGOOD_VERSION/osgood-linux-$OSGOOD_VERSION.zip"; \
	echo "$OSGOOD_SHA osgood.zip" | sha256sum -c -; \
	unzip osgood.zip; \
	chmod +x ./osgood; \
	mv ./osgood /usr/bin/osgood; \
	rm osgood.zip; \
	mkdir -p /srv/osgood; \
	chown osgood:osgood /srv/osgood

# Copies a tiny sample app
# COPY ./examples/simple/* /srv/osgood-example/
VOLUME /srv/osgood

WORKDIR /srv/osgood

EXPOSE 8080

# Users can override this when they extend the image
# CMD ["osgood", "/srv/osgood-example/app.js"]
ENTRYPOINT ["osgood"]
