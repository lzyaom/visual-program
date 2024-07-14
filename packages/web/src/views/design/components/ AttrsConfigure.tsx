import { defineComponent } from 'vue'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@components'

export default defineComponent({
  name: 'AttrsConfigure',
  setup() {
    return () => (
      <div class="attrs-configure">
        <Tabs default-value="style" class="w-full">
          <TabsList class="grid w-full grid-cols-3 bg-slate-200">
            <TabsTrigger value="style">样式</TabsTrigger>
            <TabsTrigger value="property">属性</TabsTrigger>
            <TabsTrigger value="rule">规则</TabsTrigger>
          </TabsList>
          <TabsContent value="style"> Make changes to your account here. </TabsContent>
          <TabsContent value="property"> Change your password here. </TabsContent>
          <TabsContent value="rule"> Change your password here. </TabsContent>
        </Tabs>
      </div>
    )
  }
})
