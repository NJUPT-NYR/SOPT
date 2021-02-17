import { Vue, Prop } from 'vue-property-decorator';

export default class Layout extends Vue {
  @Prop({ type: String, default: 'egg' }) title?: string;
  @Prop({ type: String, default: 'Vue TypeScript Framework, Server Side Render' }) description?: string;
  @Prop({ type: String, default: 'Vue,TypeScript,Isomorphic' }) keywords?: string;

  // isNode: boolean = EASY_ENV_IS_NODE;

  created() {
    console.log('>>EASY_ENV_IS_NODE create', EASY_ENV_IS_NODE);
  }
}