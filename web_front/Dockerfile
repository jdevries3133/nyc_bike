# this is very cursed but the build needs to occur outside docker before
# building the container, because trying to get trunk to run inside docker is
# proving to be too annoying for me.

# as a workaround, always use `make build` or `make deploy` rather than
# interacting with this dockerfile directly

FROM nginx:1.21-alpine

COPY ./dist /usr/share/nginx/html/

ENTRYPOINT ["nginx", "-g", "daemon off;"]
