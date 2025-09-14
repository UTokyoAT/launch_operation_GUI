import { useState, useEffect, useRef } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
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

  return (
    <>
      <p>{messages}</p>
    </>
  )
}

export default App
