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
        console.log("送信しました")
      })
      .catch((error) => {
        console.log("送信に失敗しました" +error)
      })
  }

  return (
    <>
      <div className="carousel">
        {logs.map((log) => (
          <div className="carousel-item card bg-secondary m-5">
            <LogView key={log.name} {...log} />
          </div>
        ))}
      </div>
      <br />
      <button className="btn btn-primary" onClick={onClick}>
        送信
      </button>
    </>
  )
}

export default App
