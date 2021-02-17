import { Vue, Component } from 'vue-property-decorator';
import Layout from 'component/layout/index.vue';

@Component({
  components: {
    Layout
  }
})

export default class Home extends Vue {}