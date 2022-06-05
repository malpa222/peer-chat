FROM node:16

WORKDIR /usr/src/frontend

ENV PATH /usr/src/frontend/node_modules/.bin:$PATH
COPY .env ./
COPY public/ public/
COPY src/ src/
COPY package.json ./
COPY package-lock.json ./
RUN npm install && \
    npm install -g serve && \
    npm run build --production

COPY . ./
EXPOSE 3000

CMD ["serve", "-s", "-d", "build"]