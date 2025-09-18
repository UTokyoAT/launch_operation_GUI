import { useState, useEffect, useRef } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import axios from 'axios'
import './App.css'
import type { Log } from './Log'
import LogView from './LogView'

function App() {
  const [logs, setLogs] = useState<Log[]>([]);
  const socketRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const websocket = new WebSocket("ws://localhost:8080/log")
    socketRef.current = websocket

    const onMessage = (event: MessageEvent<string>) => {
      const log = JSON.parse(event.data)
      setLogs(Object.entries(log).map(([name, value]: [string, any]) => ({ name, value: value.toString() })))
    }

    websocket.addEventListener("message", onMessage)

    return () => {
      websocket.close()
      websocket.removeEventListener("message", onMessage)
    }
  }, [])

  const onClick = () => {
    axios.post("http://localhost:8080/send", "data")
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
      <div className="carousel bg-primary">
        {logs.map((log) => (
          <div className="carousel-item card bg-secondary">
            <LogView key={log.name} {...log} />
          </div>
        ))}
      </div>
      <button className="btn btn-primary" onClick={onClick}>
        送信
      </button>
    </>
  )
}

export default App
