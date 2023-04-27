#!/bin/sh

POSITIONAL_ARGS=()

SH_DIR=$(dirname $0)
ENVIRONMENT="dev"

while [[ $# -gt 0 ]]; do
  case $1 in
    -e|--environment)
      ENVIRONMENT="$2"
      shift # past argument
      shift # past value
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done

set -- "${POSITIONAL_ARGS[@]}" # restore positional parameters

COMPOSE_FILE="$SH_DIR/docker-compose.${ENVIRONMENT}.yml"


echo "launching hellmut container ..."
echo "ENVIRONMENT: $ENVIRONMENT"
echo "-------------------------------------------"

echo "starting docker service..."
sudo systemctl start docker || exit 1

echo "starting docker containers..."
sudo docker compose -f "$COMPOSE_FILE" --project-directory "$SH_DIR" up --build

echo "returning to original dir..."
cd "$orig_dir" || exit 1
