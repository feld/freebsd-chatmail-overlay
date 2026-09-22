PORTNAME=		iroh
PORTREVISION=		7
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

USE_RC_SUBR=	iroh-relay

CARGO_BUILD_ARGS=	--bin iroh-relay --features=server

GROUP=	iroh
USER=	iroh

do-install:
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/iroh-relay ${STAGEDIR}${LOCALBASE}/bin
	${INSTALL_DATA} ${FILESDIR}/iroh-relay.toml.sample ${STAGEDIR}${LOCALBASE}/etc

.include <bsd.port.mk>
