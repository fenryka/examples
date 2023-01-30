import java.lang.IllegalArgumentException
import java.util.stream.Collectors

    fun <K, V> List<Pair<K, V>>.toLinkedHashMap(): LinkedHashMap<K, V> {
        return this.stream().collect(Collectors.toMap(Pair<K, V>::first, Pair<K, V>::second, { _, _ -> throw IllegalArgumentException("Duplicate keys not supported")}) { LinkedHashMap() })
    }