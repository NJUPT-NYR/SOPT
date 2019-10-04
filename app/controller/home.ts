import { Controller } from 'egg';

export default class HomeController extends Controller {
  public async index() {
    const { ctx } = this;
    const local = {
      appName: 'NYR',
      pageName: 'Home',
      title: 'NYR | Home',
    };
    await ctx.render('home.js',local);
  }
}
