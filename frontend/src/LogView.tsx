import React from 'react'
import type { Log } from './Log';

export default function LogView(props: Log) {
    const { name, value } = props;
  return (
    <div>
        <p>{name}</p>
        <p>{value}</p>
    </div>
  )
}
