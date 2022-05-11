import React from 'react'
import classNames from 'classnames'

export default function Message(props) {
    return (
        <div className={classNames({
            'flex': true,
            'justify-end': props.us,
        })}>
            <div className={classNames({
                'w-64 mx-8 my-1 px-4 py-2 bg-indigo-500 rounded-md': props.us,
                'w-64 mx-8 my-1 px-4 py-2 bg-gray-200 rounded-md': !props.us
            })}>
                <p className={classNames({
                    'text-white': props.us,
                    'break-words': true
                })}>{props.text}</p>
            </div>
        </div>
    )
}