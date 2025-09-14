import { useState, useEffect, useRef } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import axios from 'axios'
import './App.css'

function App() {
  const [messages, setMessages] = useState("");
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const websocket = new WebSocket("ws://localhost:8080/log")
    socketRef.current = websocket

    const onMessage = (event: MessageEvent<string>) => {
      setMessages(event.data)
    }
    

    websocket.addEventListener("message", onMessage)

    return () => {
      websocket.close()
      websocket.removeEventListener("message", onMessage)
    }
  }, [])

  const onClick = () => {
    axios.post("http://localhost:8080/send", "test")
      .then((_) => {
        alert("送信しました")
      })
      .catch((error) => {
        alert("送信に失敗しました" +error)
      })
  }

  return (
    <>
      <h1 className="text-3xl font-bold underline">
        Hello world!
      </h1>
      <p>{messages}</p>
      <button className="btn btn-primary" onClick={onClick}>
        送信
      </button>
    </>
  )
}

export default App
