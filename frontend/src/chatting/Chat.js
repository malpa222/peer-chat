import React, { useState } from 'react'
import classNames from 'classnames';

export default function Chat({ message, address, selected, selectChat }) {
    return (
        <div 
            className={classNames({
                "w-full bg-white my-4 px-4 py-2 bg-gray-50 bg-opacity-0 rounded-md hover:cursor-pointer": true,
                "bg-purple-500 bg-opacity-75": selected,
                "transition ease-out hover:bg-purple-500 hover:bg-opacity-25 hover:cursor-pointer duration-1000": !selected
            })}
            onClick={() => selectChat(true)}
        >
            <div className="truncate w-48"><span className="text-gray-800">{address}</span></div>
            <div><small className="text-gray-600">{message}</small></div>
        </div>
    )
}