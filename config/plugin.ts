import { EggPlugin } from 'egg';

const plugin: EggPlugin = {
  vuessr: {
    enable: true,
    package: 'egg-view-vue-ssr'
  }
};

export default plugin;
