PORTNAME=	chatmaild
PORTVERSION=	0.2
PORTREVISION=	0
DISTVERSIONSUFFIX=	.20250506
CATEGORIES=	mail python
MASTER_SITES=	LOCAL/feld
PKGNAMEPREFIX=	${PYTHON_PKGNAMEPREFIX}

MAINTAINER=	feld@FreeBSD.org
COMMENT=	Python services for Chatmail servers
WWW=		https://github.com/chatmail/relay/

WRKSRC=	${WRKDIR}/${PORTNAME}-${PORTVERSION}

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}setuptools>=42:devel/py-setuptools@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}wheel>=0:devel/py-wheel@${PY_FLAVOR}

LICENSE=	MIT

USES=		dos2unix python
USE_PYTHON=	autoplist concurrent pep517
NO_ARCH=	yes

.include <bsd.port.mk>
