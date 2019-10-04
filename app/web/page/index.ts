'use strict';
import App from '../framework/app';
import createRouter from './router/index';
import entry from './view/index.vue';
export default new App({ entry, createRouter }).bootstrap();