import React from 'react'
import type { Log } from './Log';

export default function LogView(props: Log) {
    const { name, value } = props;
  return (
    <div>
        <p className="text-lg">{name}</p>
        <p className="text-lg">{value}</p>
    </div>
  )
}
