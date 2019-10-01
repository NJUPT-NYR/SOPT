import { EggAppConfig } from 'egg';
import * as path from 'path';

export default (appInfo: EggAppConfig) => {
  const config : any = {} ;

  // override config from framework / plugin
  // use for cookie sign key, should change to your own and keep security
  config.keys = appInfo.name + '_1569900424849_4221';

  // add your egg config in here
  config.middleware = [];

  // add your special config in here
  // const bizConfig = {
  //   sourceUrl: `https://github.com/NJUPT-NYR/SOPT/`,
  // };

  config.view = {
    cache: false
  };

  config.vuessr = {
    layout: path.resolve(appInfo.baseDir, 'app/web/view/layout.html'),
    renderOptions: {
      basedir: path.join(appInfo.baseDir, 'app/view'),
    },
  };

  config.static = {
    prefix: '/public/',
    dir: path.join(appInfo.baseDir, 'public')
  };

  // the return config will combines to EggAppConfig
  return config;
};
