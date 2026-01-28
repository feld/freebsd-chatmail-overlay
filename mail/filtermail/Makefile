PORTNAME=	filtermail
DISTVERSIONPREFIX=	v
DISTVERSION=	0.2.0
CATEGORIES=	mail

MAINTAINER=	feld@FreeBSD.org
COMMENT=	Filtermail for Chatmail
WWW=		https://github.com/chatmail/filtermail

LICENSE=	MIT
LICENSE_FILE=	${WRKSRC}/LICENSE

USES=		cargo
USE_GITHUB=	yes
GH_ACCOUNT=	chatmail
#USE_RC_SUBR=	filtermail

#SUB_LIST=	USERS=${USERS} \
#		GROUPS=${GROUPS}

PLIST_FILES=	bin/filtermail

.include <bsd.port.mk>
