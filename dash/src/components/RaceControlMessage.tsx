import Image from "next/image";
import { motion } from "framer-motion";
import { utc } from "moment";
import { useEffect, useRef } from "react";

import { Message } from "@/types/state.type";
import { toTrackTime } from "@/lib/toTrackTime";

type Props = {
	msg: Message;
	utcOffset: string;
};

const loadTime = new Date();

export function RaceControlMessage({ msg, utcOffset }: Props) {
	const lastMessageIdRef = useRef<string | null>(null);

	useEffect(() => {
		const msgTime = new Date(msg.utc)
		console.log(loadTime);
		console.log(msgTime);

		if (typeof window !== "undefined") {
			if (loadTime < msgTime) {
				if (msg.id !== lastMessageIdRef.current) {
					lastMessageIdRef.current = msg.id;

					const customSettings = localStorage.getItem("custom");

					if (customSettings) {
						const parsedSettings = JSON.parse(customSettings);

						if (parsedSettings.raceControlChime === true) {
							const audio = new Audio("/sounds/chime.mp3");
							audio.play();
							console.log("played audio")
						}
					}
			  }
			}
		}
	}, [msg]);
	return (
		<motion.li animate={{ opacity: 1, y: 0 }} initial={{ opacity: 0, y: -20 }} className="flex flex-col gap-1">
			<div className="flex items-center gap-1 text-sm font-medium leading-none text-gray-500">
				{msg.lap && (
					<>
						<p>LAP {msg.lap}</p>
						{"·"}
					</>
				)}
				<time dateTime={utc(msg.utc).local().format("HH:mm:ss")}>{utc(msg.utc).local().format("HH:mm:ss")}</time>
				{"·"}
				<time className="text-gray-600" dateTime={utc(msg.utc).format("HH:mm")}>
					{utc(toTrackTime(msg.utc, utcOffset)).format("HH:mm")}
				</time>
			</div>

			<div className="flex gap-1">
				{msg.flag && msg.flag !== "CLEAR" && (
					<div>
						<Image
							src={`/flags/${msg.flag.toLowerCase().replaceAll(" ", "-")}-flag.svg`}
							alt={msg.flag}
							width={20}
							height={20}
						/>
					</div>
				)}

				<p className="text-sm">{msg.message}</p>
			</div>
		</motion.li>
	);
}
