PORTNAME=		iroh
DISTVERSIONPREFIX=	v
DISTVERSION=		0.35.0
PKGNAMESUFFIX=	-relay
CATEGORIES=		net

MAINTAINER=		feld@FreeBSD.org
COMMENT=		Relay service for iroh
WWW=			https://www.iroh.computer/

LICENSE=		MIT

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	n0-computer

USE_RC_SUBR=	iroh_relay

PLIST_FILES=	bin/iroh-relay

CARGO_BUILD_ARGS=	--bin iroh-relay --features=server

GROUP=	iroh-relay
USER=	iroh-relay

do-install:
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/iroh-relay ${STAGEDIR}${LOCALBASE}/bin

.include <bsd.port.mk>
