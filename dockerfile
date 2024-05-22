FROM node

WORKDIR /app

COPY . .

WORKDIR web

RUN npm install

EXPOSE 8080/udp
EXPOSE 8080/tcp

CMD ["npm", "run", "serve"]
