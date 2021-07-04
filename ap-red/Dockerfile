# This Dockerfile will be used when we begin moving away from
# the more monolithic architecture used as our system
# architectural starting point. Since this service encapsulates
# Redis, a networking server/client which runs continuously
# and communicates with other services in order to perform its
# task in the architecture, it is a clear choice to be broken out
# into its own container in the Devisa API system.
#
FROM docker.io/redis:latest

RUN ["./entrypoint.sh"]
