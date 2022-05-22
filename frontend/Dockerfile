FROM node:16

WORKDIR /usr/src/frontend

ENV PATH /usr/src/frontend/node_modules/.bin:$PATH

COPY package.json ./
COPY package-lock.json ./
RUN npm install

COPY . ./
EXPOSE 3000

CMD ["npm", "start"]