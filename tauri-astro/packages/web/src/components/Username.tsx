import { get_username } from "t-web";
import { defineComponent } from "vue";

export default defineComponent({
    name: "Username",
    props: { name: { type: String, required: true } },
    async setup(props) {
        const { name } = props;

        const user = await get_username(name)
        console.log(user)

        return () => <div>{user}</div>
    }
})
