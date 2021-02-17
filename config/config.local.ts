import { EggAppConfig } from 'egg';
import * as path from 'path';

export default (appInfo: EggAppConfig) => {
  const exports: any = {};

  exports.static = {
    maxAge: 0
  };

  exports.development = {
    watchDirs: ['build'],
    ignoreDirs: ['app/web', 'public', 'config']
  };

  exports.logview = {
    dir: path.join(appInfo.baseDir, 'logs')
  };

  exports.vuessr = {
    injectCss: false
  };

  return exports;
};