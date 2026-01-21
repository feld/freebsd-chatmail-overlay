PORTNAME=		chatmail
DISTVERSIONPREFIX=	v
DISTVERSION=		0.3
PKGNAMESUFFIX=	-turn
CATEGORIES=		net

MAINTAINER=		feld@FreeBSD.org
COMMENT=		TURN server for chatmail relays
WWW=			https://github.com/chatmail/chatmail-turn

LICENSE=		BSD0CLAUSE

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	chatmail
GH_PROJECT=	chatmail-turn

USE_RC_SUBR=	chatmail_turn

PLIST_FILES=	bin/chatmail-turn

#GROUP=	iroh-relay
#USER=	iroh-relay

do-install:
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/chatmail-turn ${STAGEDIR}${LOCALBASE}/bin

.include <bsd.port.mk>
