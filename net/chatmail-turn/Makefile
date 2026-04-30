PORTNAME=		chatmail
DISTVERSIONPREFIX=	v
DISTVERSION=		0.4
PORTREVISION=		0
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

#GROUP=	vmail
#USER=	vmail

do-install:
	${INSTALL_PROGRAM} ${CARGO_TARGET_DIR}/*/chatmail-turn ${STAGEDIR}${LOCALBASE}/bin

.include <bsd.port.mk>
