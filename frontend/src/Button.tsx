import axios from 'axios'
export default function Button(props: { name: string }) {
    const { name } = props;

    const onClick = () => {
        axios.post("http://localhost:8080/send", name)
            .then((_) => {
                console.log("送信しました")
            })
            .catch((error) => {
                console.error("送信に失敗しましました" +error)
            })
    };
  return (
    <button className="btn btn-primary m-5" onClick={onClick}>
        {name}
    </button>
  )
}
